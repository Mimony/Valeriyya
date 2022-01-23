import { 
    AudioResource, 
    createAudioResource, 
    AudioPlayer,
	AudioPlayerStatus,
	createAudioPlayer,
	VoiceConnection,
	VoiceConnectionDisconnectReason,
	VoiceConnectionStatus,
	AudioPlayerState,
	VoiceConnectionState,
} from '@discordjs/voice';
import play from "play-dl"
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


	public async createAudioResource(): Promise<AudioResource<Track>> {
		let { stream , type } = await play.stream(this.url);

		return createAudioResource(stream, { inputType: type, metadata: this, inlineVolume: true })
	}


	public static async from(url: string, requestedBy: User, channel: TextBasedChannel | null, guildid: string, looping: boolean = false, method: Pick<Track, 'onStart' | 'onFinish' | 'onError'>): Promise<Track> {
		const info = await play.video_info(url);

		const methods = {
			onStart() {
				method.onStart();
			},
			onFinish() {
				method.onFinish();
			},
			onError(error: Error) {
				method.onError(error);
			},
		};

		return new Track({
			title: info.video_details.title!,
			requestedBy,
			channel,
			guildid,
			looping,
			url,
			...methods,
		});
	}
}

import { promisify } from 'node:util';

const wait = promisify(setTimeout);

export class MusicSubscription {
	public readonly voiceConnection: VoiceConnection;
	public readonly audioPlayer: AudioPlayer;
	public queue: Track[];
	public queueLoop: boolean;
	public currentPlaying: Track | null;

	public queueLock = false;
	public readyLock = false;

	public constructor(voiceConnection: VoiceConnection) {
		this.voiceConnection = voiceConnection;
		this.audioPlayer = createAudioPlayer();
		this.queue = [];
		this.queueLoop = false;
		this.currentPlaying = null;

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
					this.currentPlaying = (newState.resource as AudioResource<Track>).metadata
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
		if (this.queueLock || this.audioPlayer.state.status !== AudioPlayerStatus.Idle) {
			return;
		}
		// Lock the queue to guarantee safe access
		this.queueLock = true;

		if (this.currentPlaying?.looping){
			this.queue.unshift(this.currentPlaying);
		}

		if (this.queueEmpty()) return;

		let nextTrack;
		if (this.queueLoop){
			this.queue.splice(this.queue.length, 0, this.currentPlaying as Track);
			nextTrack = this.queue.shift();
		} else {
			nextTrack = this.queue.shift();
		}
		
		if (nextTrack == null || nextTrack == undefined) return;

		try {
			// Attempt to convert the Track into an AudioResource (i.e. start streaming the video)
			const resource = await nextTrack!.createAudioResource();
			this.audioPlayer.play(resource);
			this.queueLock = false;
		} catch (error) {
			// If an error occurred, try the next item of the queue instead
			nextTrack!.onError(error as Error);
			this.queueLock = false;
			return this.processQueue();
		}
	}

	public queueEmpty():boolean {
		if (this.currentPlaying?.looping) return false;
		return !this.queue.length;
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