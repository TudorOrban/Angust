import { Injectable } from '@angular/core';
import { UIItem } from '../../../shared/types';
import {
    faAddressBook,
    faCode,
    faUser,
} from '@fortawesome/free-solid-svg-icons';
import { Router } from '@angular/router';
import { VersionService } from '../../version/services/version.service';

@Injectable({
    providedIn: 'root',
})
export class MainNavigationService {

    constructor(
        private readonly router: Router,
        private readonly versionService: VersionService
    ) {}

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
        console.log("set active item to ", value);
        this.activeItemValue = value;
        
        let link = `${this.versionService.getActiveVersion()}/${value}`;

        this.router.navigate([link]);
    }

    getActiveItem() {
        return this.activeItemValue;
    }
}
