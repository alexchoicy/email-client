export type EmailAccount = {
    id: string;
    email: string;
    provider: EmailProvider;
    icon?: string;
    displayName?: string;
    avatarUrl?: string;
    isActive?: boolean;
    lastChecked?: Date;
    unreadCount?: number;
    error?: string; // For error handling
}

export enum EmailProvider {
    All,
    Gmail,
    Outlook
}