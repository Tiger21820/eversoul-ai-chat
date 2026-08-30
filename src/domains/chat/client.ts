import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { invokeCommand, tauriCommands } from '../../shared/api';
import { ChatRoom, ChatMessage, ChatStreamDoneEvent, ChatStreamTokenEvent } from './types';
export const CHAT_STREAM_TOKEN_EVENT = 'chat-stream-token';
export const CHAT_STREAM_DONE_EVENT = 'chat-stream-done';
export const chatClient = {
    async createRoom(title: string): Promise<ChatRoom> {
        return invokeCommand<ChatRoom>(tauriCommands.chat.createRoom, { title });
    },
    async createSessionRoom(title: string, personaId: string): Promise<ChatRoom> {
        return invokeCommand<ChatRoom>(tauriCommands.chat.createSessionRoom, {
            title,
            persona_id: personaId,
        });
    },
    async getEverTalkSessionRoom(): Promise<ChatRoom> {
        return invokeCommand<ChatRoom>(tauriCommands.chat.getEverTalkSessionRoom);
    },
    async getLatestSessionRoom(personaId: string): Promise<ChatRoom | null> {
        return invokeCommand<ChatRoom | null>(tauriCommands.chat.getLatestSessionRoom, {
            persona_id: personaId,
        });
    },
    async listRooms(): Promise<ChatRoom[]> {
        return invokeCommand<ChatRoom[]>(tauriCommands.chat.listRooms);
    },
    async listRoomsForPersona(personaId: string): Promise<ChatRoom[]> {
        return invokeCommand<ChatRoom[]>(tauriCommands.chat.listRoomsForPersona, {
            persona_id: personaId,
        });
    },
    async startNewRoom(personaId: string): Promise<ChatRoom> {
        return invokeCommand<ChatRoom>(tauriCommands.chat.startNewRoom, {
            persona_id: personaId,
        });
    },
    async deleteRoom(roomId: string): Promise<void> {
        return invokeCommand<void>(tauriCommands.chat.deleteRoom, { room_id: roomId });
    },
    async deleteMessage(messageId: string): Promise<void> {
        return invokeCommand<void>(tauriCommands.chat.deleteMessage, { message_id: messageId });
    },
    async listMessages(roomId: string): Promise<ChatMessage[]> {
        return invokeCommand<ChatMessage[]>(tauriCommands.chat.listMessages, { room_id: roomId });
    },
    async listMessagesForPersona(roomId: string, personaId: string): Promise<ChatMessage[]> {
        return invokeCommand<ChatMessage[]>(tauriCommands.chat.listMessagesForPersona, {
            room_id: roomId,
            persona_id: personaId,
        });
    },
    async preparePersonaCache(personaId: string): Promise<boolean> {
        return invokeCommand<boolean>(tauriCommands.chat.preparePersonaCache, {
            persona_id: personaId,
        });
    },
    async sendMessage(
        roomId: string,
        content: string,
        personaId: string,
        requestId: string,
    ): Promise<ChatMessage> {
        return invokeCommand<ChatMessage>(tauriCommands.chat.sendMessage, {
            room_id: roomId,
            content,
            persona_id: personaId,
            request_id: requestId,
        });
    },
    async onStreamToken(
        requestId: string,
        handler: (token: string) => void,
    ): Promise<UnlistenFn> {
        return listen<ChatStreamTokenEvent>(CHAT_STREAM_TOKEN_EVENT, (event) => {
            if (event.payload.request_id === requestId) {
                handler(event.payload.token);
            }
        });
    },
    async onStreamDone(
        requestId: string,
        handler: (event: ChatStreamDoneEvent) => void,
    ): Promise<UnlistenFn> {
        return listen<ChatStreamDoneEvent>(CHAT_STREAM_DONE_EVENT, (event) => {
            if (event.payload.request_id === requestId) {
                handler(event.payload);
            }
        });
    },
};
