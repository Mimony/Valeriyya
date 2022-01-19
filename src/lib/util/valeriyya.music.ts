import { getInfo } from 'ytdl-core';
import { 
    AudioResource, 
    createAudioResource, 
    demuxProbe, 
    AudioPlayer,
	AudioPlayerStatus,
	createAudioPlayer,
	VoiceConnection,
	VoiceConnectionDisconnectReason,
	VoiceConnectionStatus,
	AudioPlayerState,
	VoiceConnectionState
} from '@discordjs/voice';
import { raw as ytdl } from 'youtube-dl-exec';
import type { TextBasedChannel, User } from 'discord.js';

export interface TrackData {
	url: string;
	title: string;
	requestedBy: User;
	looping: boolean;
	channel: TextBasedChannel | null;
	guildid: string|null;
	onStart: () => void;
	onFinish: () => void;
	onError: (error: Error) => void;
}

// eslint-disable-next-line @typescript-eslint/no-empty-function
const noop = () => {};

export class Track implements TrackData {
	public readonly url: string;
	public readonly title: string;
	public readonly requestedBy: User;
	public looping: boolean;
	public channel: TextBasedChannel | null
	public guildid: string | null;
	public readonly onStart: () => void;
	public readonly onFinish: () => void;
	public readonly onError: (error: Error) => void;

	public constructor({ url, title, requestedBy, looping, channel, guildid, onStart, onFinish, onError }: TrackData) {
		this.url = url;
		this.title = title;
		this.requestedBy = requestedBy;
		this.looping = looping;
		this.channel = channel;
		this.guildid = guildid;
		this.onStart = onStart;
		this.onFinish = onFinish;
		this.onError = onError;
	}


	public createAudioResource(): Promise<AudioResource<Track>> {
		return new Promise((resolve, reject) => {
			const process = ytdl(
				this.url,
				{
					o: '-',
					q: '',
					f: 'bestaudio[ext=webm+acodec=opus+asr=48000]/bestaudio',
					r: '100K',
				},
				{ stdio: ['ignore', 'pipe', 'ignore'] },
			);
			if (!process.stdout) {
				reject(new Error('No stdout'));
				return;
			}
			const stream = process.stdout;
			const onError = (error: Error) => {
				if (!process.killed) process.kill();
				stream.resume();
				reject(error);
			};
			process.once('spawn', () => {
					demuxProbe(stream)
						.then((probe) => resolve(createAudioResource(probe.stream, { metadata: this, inputType: probe.type, inlineVolume: true })))
						.catch(onError);
				})
				.catch(onError);
		});
	}

	/**
	 * Creates a Track from a video URL and lifecycle callback methods.
	 *
	 * @param url The URL of the video
	 * @param methods Lifecycle callbacks
	 * @returns The created Track
	 */
	public static async from(url: string, requestedBy: User, channel: TextBasedChannel | null, guildid: string, looping: boolean = false, methods: Pick<Track, 'onStart' | 'onFinish' | 'onError'>): Promise<Track> {
		const info = await getInfo(url);

		// The methods are wrapped so that we can ensure that they are only called once.
		const wrappedMethods = {
			onStart() {
				wrappedMethods.onStart = noop;
				methods.onStart();
			},
			onFinish() {
				wrappedMethods.onFinish = noop;
				methods.onFinish();
			},
			onError(error: Error) {
				wrappedMethods.onError = noop;
				methods.onError(error);
			},
		};

		return new Track({
			title: info.videoDetails.title,
			requestedBy,
			channel,
			guildid,
			looping,
			url,
			...wrappedMethods,
		});
	}
}

import { promisify } from 'node:util';

const wait = promisify(setTimeout);

export class MusicSubscription {
	public readonly voiceConnection: VoiceConnection;
	public readonly audioPlayer: AudioPlayer;
	public queue: Track[];
	public queueLock = false;
	public readyLock = false;

	public constructor(voiceConnection: VoiceConnection) {
		this.voiceConnection = voiceConnection;
		this.audioPlayer = createAudioPlayer();
		this.queue = [];

		this.voiceConnection.on('stateChange', async (_, newState) => {
				if (newState.status === VoiceConnectionStatus.Disconnected) {
					if (newState.reason === VoiceConnectionDisconnectReason.WebSocketClose && newState.closeCode === 4014) {
						try {
							await waitForResourceToEnterState(this.voiceConnection, VoiceConnectionStatus.Connecting, 5_000);
						} catch {
							this.voiceConnection.destroy();
						}
					} else if (this.voiceConnection.rejoinAttempts < 5) {
						await wait((this.voiceConnection.rejoinAttempts + 1) * 5_000);
						this.voiceConnection.rejoin();
					} else {
						this.voiceConnection.destroy();
					}
				} else if (newState.status === VoiceConnectionStatus.Destroyed) {
					this.stop();
				} else if (
					!this.readyLock &&
					(newState.status === VoiceConnectionStatus.Connecting || newState.status === VoiceConnectionStatus.Signalling)
				) {
					this.readyLock = true;
					try {
						await waitForResourceToEnterState(this.voiceConnection, VoiceConnectionStatus.Ready, 20_000);
					} catch {
						if (this.voiceConnection.state.status !== VoiceConnectionStatus.Destroyed) this.voiceConnection.destroy();
					} finally {
						this.readyLock = false;
					}
				}
			},
		);

		this.audioPlayer.on('stateChange', async (oldState: AudioPlayerState, newState: AudioPlayerState) => {
				if (newState.status === AudioPlayerStatus.Idle && oldState.status !== AudioPlayerStatus.Idle) {
					(oldState.resource as AudioResource<Track>).metadata.onFinish();
					void this.processQueue();
				} else if (newState.status === AudioPlayerStatus.Playing) {
					(newState.resource as AudioResource<Track>).metadata.onStart();
				}
			},
		);

		this.audioPlayer.on('error', (error: { resource: any }) =>
		// @ts-ignore
			(error.resource as AudioResource<Track>).metadata.onError(error),
		);

		voiceConnection.subscribe(this.audioPlayer);
	}

	public enqueue(track: Track) {
		this.queue.push(track);
		void this.processQueue();
	}

	public stop() {
		this.queueLock = true;
		this.queue = [];
		this.audioPlayer.stop(true);
	}

	private async processQueue(): Promise<void> {
		if (this.queueLock || this.audioPlayer.state.status !== AudioPlayerStatus.Idle || this.queue.length === 0) {
			return;
		}
		this.queueLock = true;

		const nextTrack = this.queue.shift()!;
		try {
			const resource = await nextTrack.createAudioResource();
			this.audioPlayer.play(resource);
			this.queueLock = false;
		} catch (error) {
			nextTrack.onError(error as Error);
			this.queueLock = false;
			return this.processQueue();
		}
	}
}

export function waitForResourceToEnterState(resource: VoiceConnection, status: VoiceConnectionStatus, timeoutMS: number): Promise<void>;
export function waitForResourceToEnterState(resource: AudioPlayer, status: AudioPlayerStatus, timeoutMS: number): Promise<void>;
export function waitForResourceToEnterState(resource: VoiceConnection | AudioPlayer, status: VoiceConnectionStatus | AudioPlayerStatus, timeoutMS: number): Promise<void> {
	return new Promise((res, rej) => {
		if (resource.state.status === status) res(void 0);
		let timeout: NodeJS.Timeout | undefined = undefined;
		function onStateChange(_: VoiceConnectionState | AudioPlayerState, newState: VoiceConnectionState | AudioPlayerState) {
			if (newState.status !== status) return;
			if (timeout) clearTimeout(timeout);
			(resource as AudioPlayer).removeListener("stateChange", onStateChange);
			return res(void 0);
		}
		(resource as AudioPlayer).on("stateChange", onStateChange);
		timeout = setTimeout(() => {
			(resource as AudioPlayer).removeListener("stateChange", onStateChange);
			rej(new Error("Didn't enter state in time"));
		}, timeoutMS);
	});
}