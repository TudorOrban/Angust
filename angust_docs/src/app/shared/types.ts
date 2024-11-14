import { IconDefinition } from "@fortawesome/fontawesome-svg-core";

export interface PageMetadata {
    activeVersion: string;
    activeMainItemValue: string;
    activeSecondaryItemValue: string;
}


export interface UIItem {
    label: string;
    value: string;
    link?: string;
    icon?: IconDefinition;
    subItems?: UIItem[];
}