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
                label: 'HTML and CSS',
                value: 'html-and-css',
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
                    {
                        label: 'HTML Templates',
                        value: 'html-templates',   
                    },
                    {
                        label: 'Component Functions',
                        value: 'component-functions',
                    },
                    {
                        label: 'Component Inputs',
                        value: 'component-inputs',
                    },
                    {
                        label: 'Component Outputs',
                        value: 'component-outputs',
                    },
                    {
                        label: 'Lifecycle Hooks',
                        value: 'lifecycle-hooks',
                    }
                ]
            },
            {
                label: 'Directives',
                value: 'directives',
                subItems: [
                    {
                        label: 'Overview',
                        value: 'overview',
                    },
                    {
                        label: 'If Directive',
                        value: 'if-directive',
                    },
                    {
                        label: 'For Directive',
                        value: 'for-directive',
                    },
                    {
                        label: 'On Click Directive',
                        value: 'on-click-directive',
                    }
                ]
            },
            {
                label: 'Services',
                value:'services',
                subItems: [
                    {
                        label: 'Overview',
                        value: 'overview',
                    },
                    {
                        label: 'Registration and Usage',
                        value:'registration',
                    },
                    {
                        label: 'Async Operations',
                        value: 'async-operations',
                    }
                ]
            },
            {
                label: 'Routing',
                value: 'routing',
                subItems: [
                    {
                        label: 'Overview',
                        value: 'overview',
                    },
                    {
                        label: 'Routing Configuration',
                        value: 'routing-configuration',
                    },
                    {
                        label: 'Router API',
                        value: 'router-api',
                    },
                ]
            },
            {
                label: 'Angust CLI',
                value: 'angust-cli',
                subItems: [
                    {
                        label: 'Overview',
                        value: 'overview',
                    },
                    {
                        label: 'Installation',
                        value: 'installation',
                    },
                    {
                        label: 'Create Project',
                        value: 'create-project',
                    },
                    {
                        label: 'Generate Component',
                        value: 'generate-component',
                    },
                    {
                        label: 'Generate Service',
                        value: 'generate-service',
                    }
                ]
            },
            {
                label: 'Best Practices',
                value: 'best-practices',
            },
            {
                label: 'Roadmap',
                value: 'roadmap',
            },
            {
                label: 'Troubleshooting',
                value: 'troubleshooting',
            }
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