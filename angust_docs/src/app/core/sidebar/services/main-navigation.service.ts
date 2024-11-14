import { Injectable } from '@angular/core';
import { UIItem } from '../../../shared/types';
import {
    faAddressBook,
    faCode,
    faUser,
} from '@fortawesome/free-solid-svg-icons';

@Injectable({
    providedIn: 'root',
})
export class MainNavigationService {

    private readonly navItems: UIItem[] = [
        {
            label: 'User Guide',
            value: 'user-guide',
            icon: faUser,
        },
        {
            label: 'Contributor Guide',
            value: 'contributor-guide',
            icon: faCode,
        },
        {
            label: 'API Reference',
            value: 'api-reference',
            icon: faAddressBook,
        },
    ];
    activeItemValue = 'user-guide';

    getNavItems() {
        return this.navItems;
    }

    setActiveItem(value: string) {
        this.activeItemValue = value;
    }

    getActiveItem() {
        return this.activeItemValue;
    }
}
