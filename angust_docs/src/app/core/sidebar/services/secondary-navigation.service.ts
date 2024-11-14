import { Injectable } from '@angular/core';
import { UIItem } from '../../../shared/types';
import { VersionService } from './version.service';
import { Router } from '@angular/router';

@Injectable({
    providedIn: 'root',
})
export class SecondaryNavigationService {

    constructor(
        private readonly router: Router,
        private readonly versionService: VersionService
    ) {}

    private readonly navItems: Record<string, Record<string, UIItem[]>> = {
        "v1": {
            "user-guide": [
                {
                    label: 'What is Angust?',
                    value: 'overview',
                },
                {
                    label: 'Getting Started',
                    value: 'getting-started',
                },
                {
                    label: 'Components',
                    value: 'components',
                    subItems: [
                        {
                            label: 'Overview',
                            value: 'overview',
                        },
                        {
                            label: 'Component State',
                            value: 'component-state',
                        },
                    ]
                },
            ],
            "contributor-guide": [
                {
                    label: 'What is Angust?',
                    value: 'overview',
                }
            ]
        },
        "v2": {
            "user-guide": [
                {
                    label: 'What is Angust2?',
                    value: 'overview2',
                },
                {
                    label: 'Getting Started',
                    value: 'getting-started',
                },
                {
                    label: 'Components',
                    value: 'components',
                    subItems: [
                        {
                            label: 'Overview',
                            value: 'overview',
                        },
                        {
                            label: 'Component State',
                            value: 'component-state',
                        },
                    ]
                },
            ],
            "contributor-guide": [
                {
                    label: 'What is Angust?',
                    value: 'overview',
                }
            ]
        }
    };
    private activeNavItems: UIItem[] = [];
    private activeItemValue = 'overview';

    getNavItems() {
        return this.navItems;
    }

    getActiveNavItems() {
        return this.activeNavItems;
    }

    setActiveNavItems(version: string, mainItem: string) {
        console.log("setActiveNavItems: ", version, mainItem);
        const navItems = this.navItems?.[version]?.[mainItem];
        console.log("navItems: ", navItems);
        this.activeNavItems = navItems ?? [];
    }

    getActiveItem() {
        return this.activeItemValue;
    }

    setActiveItem(value: string) {
        this.activeItemValue = value;
    }

}
