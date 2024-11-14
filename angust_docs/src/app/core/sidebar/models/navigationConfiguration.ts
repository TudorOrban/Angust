import { UIItem } from "../../../shared/types";

export const secondaryNavItems: Record<string, Record<string, UIItem[]>> = {
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