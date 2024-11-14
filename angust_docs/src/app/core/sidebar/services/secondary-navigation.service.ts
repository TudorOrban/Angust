import { Injectable } from '@angular/core';
import { UIItem } from '../../../shared/types';

@Injectable({
    providedIn: 'root',
})
export class SecondaryNavigationService {

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
    private activeItemValue = 'overview';

    getNavItems() {
        return this.navItems;
    }

    getActiveItem() {
        return this.activeItemValue;
    }

    setActiveItem(value: string) {
        this.activeItemValue = value;
    }

}
