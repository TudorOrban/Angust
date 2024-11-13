import { IconDefinition } from "@fortawesome/fontawesome-svg-core";

export interface UIItem {
    label: string;
    value: string;
    link?: string;
    icon?: IconDefinition;
}