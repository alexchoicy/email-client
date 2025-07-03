import { EmailProvider } from './types/account';


// Import the SVG files
import gmailIcon from '$lib/assets/icons/providers/gmail.svg';
import outlookIcon from '$lib/assets/icons/providers/outlook.svg';

export function getProviderIcon(provider: EmailProvider): string {
    const iconMap: Record<EmailProvider, string> = {
        [EmailProvider.All]: '',
        [EmailProvider.Gmail]: gmailIcon,
        [EmailProvider.Outlook]: outlookIcon,
    };
    
    return iconMap[provider] || gmailIcon; // fallback
}

export function getProviderName(provider: EmailProvider): string {
    const nameMap: Record<EmailProvider, string> = {
        [EmailProvider.All]: 'All Accounts',
        [EmailProvider.Gmail]: 'Gmail',
        [EmailProvider.Outlook]: 'Outlook',
    };
    
    return nameMap[provider] || 'Unknown';
}