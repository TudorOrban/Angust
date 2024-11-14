import { Injectable } from '@angular/core';
import { UIItem } from '../../../shared/types';
import { secondaryNavItems } from '../models/navigationConfiguration';

@Injectable({
    providedIn: 'root',
})
export class SecondaryNavigationService {

    private readonly navItems: Record<string, Record<string, UIItem[]>> = secondaryNavItems;
    private activeItemValue = 'overview';
    private activeSubItemValue?: string;

    getNavItems(): Record<string, Record<string, UIItem[]>> {
        return this.navItems;
    }

    getActiveItem(): string {
        return this.activeItemValue;
    }

    getActiveSubItem(): string | undefined {
        return this.activeSubItemValue;
    }

    setActiveItem(value: string): void {
        this.activeItemValue = value;
    }

    setActiveSubItem(value?: string): void {
        this.activeSubItemValue = value;
    }

}
