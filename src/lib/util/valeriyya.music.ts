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
import play from "play-dl";
import { isNullish } from './valeriyya.types';
import type { TextBasedChannel, User } from 'discord.js';

export interface TrackData {
	url: string;
	title: string;
	requestedBy: User;
	looping: boolean;
	channel: TextBasedChannel | null;
	guildid: string | null;
	onStart: () => void;
	onError: (error: Error) => void;
}

export class Track implements TrackData {
	public readonly url: string;
	public readonly title: string;
	public readonly requestedBy: User;
	public looping: boolean;
	public channel: TextBasedChannel | null;
	public guildid: string;
	public readonly onStart: () => void;
	public readonly onError: (error: Error) => void;

	public constructor({ url, title, requestedBy, looping, channel, guildid, onStart, onError }: TrackData) {
		this.url = url;
		this.title = title;
		this.requestedBy = requestedBy;
		this.looping = looping;
		this.channel = channel;
		this.guildid = guildid!;
		this.onStart = onStart;
		this.onError = onError;
	}


	public async createAudioResource (): Promise<AudioResource<Track>> {
		let { stream, type } = await play.stream(this.url);

		return createAudioResource(stream, { inputType: type, metadata: this, inlineVolume: true });
	}


	public static async from (url: string, requestedBy: User, channel: TextBasedChannel | null, guildid: string, looping: boolean = false, method: Pick<Track, 'onStart' | 'onError'>): Promise<Track> {
		const info = await play.video_info(url);

		const methods = {
			onStart () {
				method.onStart();
			},
			onError (error: Error) {
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
import type { Valeriyya } from '../valeriyya.client';

const wait = promisify(setTimeout);

export class MusicSubscription {
	public client: Valeriyya;
	public readonly voiceConnection: VoiceConnection;
	public readonly guildId: string;
	public readonly audioPlayer: AudioPlayer;
	public queue: Track[];
	public queueLoop: boolean;
	public currentPlaying: Track | null;

	public queueLock: boolean = false;
	public readyLock: boolean = false;
	public disconnected: boolean = false;

	public constructor({ client, guildId }: { client: Valeriyya, guildId: string } ,voiceConnection: VoiceConnection) {
		this.client = client;
		this.guildId = guildId;
		this.voiceConnection = voiceConnection;
		this.audioPlayer = createAudioPlayer();
		this.queue = [];
		this.queueLoop = false;
		this.currentPlaying = null;
		this.disconnected = false;

		this.voiceConnection.on('stateChange', async (_: VoiceConnectionState, newState: VoiceConnectionState) => {
			if (newState.status === VoiceConnectionStatus.Disconnected) {
				if (newState.reason === VoiceConnectionDisconnectReason.WebSocketClose && newState.closeCode === 4014) {
					try {
						await Promise.race([
							waitForResourceToEnterState(this.voiceConnection, VoiceConnectionStatus.Signalling, 5000),
							waitForResourceToEnterState(this.voiceConnection, VoiceConnectionStatus.Connecting, 5000)
						]);
					} catch {
						this.voiceConnection.destroy();
						this.disconnected = true;
					}
				} else if (this.voiceConnection.rejoinAttempts < 5) {
					await wait((this.voiceConnection.rejoinAttempts + 1) * 5_000);
					this.voiceConnection.rejoin();
				} else {
					this.voiceConnection.destroy();
					this.disconnected = true;
				}
			} else if (newState.status === VoiceConnectionStatus.Destroyed) {
				this.stop();
				this.disconnected = true;
			} else if (
				!this.readyLock &&
				(newState.status === VoiceConnectionStatus.Connecting || newState.status === VoiceConnectionStatus.Signalling)
			) {
				this.readyLock = true;
				try {
					await waitForResourceToEnterState(this.voiceConnection, VoiceConnectionStatus.Ready, 20000);
				} catch {
					if (this.voiceConnection.state.status !== VoiceConnectionStatus.Destroyed) {
						this.voiceConnection.destroy();
						this.disconnected = true;
					}
				} finally {
					this.readyLock = false;
				}
			}
			if (this.disconnected) {
				this.client.subscription.delete(this.guildId)
			}
		});


		this.audioPlayer.on('stateChange', async (oldState: AudioPlayerState, newState: AudioPlayerState) => {
			if (newState.status === AudioPlayerStatus.Idle && oldState.status !== AudioPlayerStatus.Idle) {
				void this.processQueue();
			} else if (newState.status === AudioPlayerStatus.Playing) {
				if (this.currentPlaying?.url === (newState.resource as AudioResource<Track>).metadata.url) return;
				(newState.resource as AudioResource<Track>).metadata.onStart();
				this.currentPlaying = (newState.resource as AudioResource<Track>).metadata;
			}
		},
		);

		this.audioPlayer.on('error', (error: Error) =>
			(error as unknown as AudioResource<Track>).metadata.onError(error),
		);

		this.voiceConnection.subscribe(this.audioPlayer);
	}

	public enqueue (track: Track) {
		this.queue.push(track);
		void this.processQueue();
	}

	public stop () {
		this.queueLock = true;
		this.queue = [];
		this.audioPlayer.stop(true);
	}

	private async processQueue (): Promise<void> {
		if (this.queueLock || this.audioPlayer.state.status !== AudioPlayerStatus.Idle) {
			return;
		}
		this.queueLock = true;

		if (this.currentPlaying?.looping) {
			this.queue.unshift(this.currentPlaying);
		}

		if (this.queueEmpty()) return;

		let nextTrack;
		if (this.queueLoop) {
			this.queue.splice(this.queue.length, 0, this.currentPlaying as Track);
			nextTrack = this.queue.shift();
		} else {
			nextTrack = this.queue.shift();
		}

		if (isNullish(nextTrack)) return;

		try {
			const resource = await nextTrack!.createAudioResource();
			this.audioPlayer.play(resource);
			this.queueLock = false;
		} catch (error) {
			nextTrack!.onError(error as Error);
			this.queueLock = false;
			return this.processQueue();
		}
	}

	public queueEmpty (): boolean {
		if (this.currentPlaying?.looping) return false;
		return !this.queue.length;
	}
}

export function waitForResourceToEnterState (resource: VoiceConnection, status: VoiceConnectionStatus, timeoutMS: number): Promise<void>;
export function waitForResourceToEnterState (resource: AudioPlayer, status: AudioPlayerStatus, timeoutMS: number): Promise<void>;
export function waitForResourceToEnterState (resource: VoiceConnection | AudioPlayer, status: VoiceConnectionStatus | AudioPlayerStatus, timeoutMS: number): Promise<void> {
	return new Promise((res, rej) => {
		if (resource.state.status === status) res(void 0);
		let timeout: NodeJS.Timeout | undefined = undefined;
		function onStateChange (_: VoiceConnectionState | AudioPlayerState, newState: VoiceConnectionState | AudioPlayerState) {
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