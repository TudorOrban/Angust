import { Injectable } from "@angular/core";
import { Router } from "@angular/router";
import { VersionService } from "./version.service";
import { SecondaryNavigationService } from "./secondary-navigation.service";
import { MainNavigationService } from "./main-navigation.service";
import { NavigationItemType } from "../models/navigation";
import { UIItem } from "../../../shared/types";
import { BehaviorSubject } from "rxjs";

/*
 * Service responsible for managing the three navigation directories.
 */
@Injectable({
    providedIn: 'root',
})
export class NavigationManagerService {

    // Observable to efficiently update the UI, as secondary nav items depend on version and main nav item
    private readonly activeNavItemsSource = new BehaviorSubject<UIItem[]>([]);

    constructor(
        private readonly router: Router,
        private readonly versionService: VersionService,
        private readonly mainNavigationService: MainNavigationService,
        private readonly secondaryNavigationService: SecondaryNavigationService
    ) {
        const initialItems = this.secondaryNavigationService.getNavItems()["v0"]["user-guide"];
        this.activeNavItemsSource.next(initialItems);
    }

    public navigateTo(type: NavigationItemType, itemValue: string, itemSubValue?: string): void {
        const version = this.versionService.getActiveVersion();
        const mainItem = this.mainNavigationService.getActiveItem();
        
        let link;

        switch (type) {
            case NavigationItemType.Version: {
                link = this.handleVersionNavigation(itemValue, mainItem);
                break;
            }
            case NavigationItemType.MainItem: {
                link = this.handleMainItemNavigation(version, itemValue);
                break;
            }
            case NavigationItemType.SecondaryItem:
                link = this.handleSecondaryItemNavigation(version, mainItem, itemValue, itemSubValue);
                break;
            default:
                break;
        }

        if (!link) {
            console.error("Invalid link type: ", type);
            return;
        }
        this.router.navigate([link]);
    }

    private handleVersionNavigation(newVersion: string, mainItem: string): string | undefined {
        this.versionService.setActiveVersion(newVersion);

        // Ensure to update the active secondary navigation item to the first one when changing the version.
        const firstSecondaryItem = this.secondaryNavigationService.getNavItems()?.[newVersion]?.[mainItem]?.[0];
        if (firstSecondaryItem) {
            this.secondaryNavigationService.setActiveItem(firstSecondaryItem.value);
            this.secondaryNavigationService.setActiveSubItem();
            this.setActiveNavItems(newVersion, mainItem);
            return `${newVersion}/${mainItem}/${firstSecondaryItem.value}`;
        } else {
            console.log("No secondary items for version: ", newVersion);
            return undefined;
        }
    }

    private handleMainItemNavigation(version: string, newMainItem: string): string | undefined {
        this.mainNavigationService.setActiveItem(newMainItem);
                
        // Ensure to update the active secondary navigation item to the first one when changing the main item.
        const firstSecondaryItem = this.secondaryNavigationService.getNavItems()?.[version]?.[newMainItem]?.[0];
        if (firstSecondaryItem) {
            this.secondaryNavigationService.setActiveItem(firstSecondaryItem.value);
            this.secondaryNavigationService.setActiveSubItem();
            this.setActiveNavItems(version, newMainItem);
            return `${version}/${newMainItem}/${firstSecondaryItem.value}`;
        } else {
            console.log("No secondary items for main item: ", newMainItem);
            return undefined;
        }
    }

    private handleSecondaryItemNavigation(
        version: string, 
        mainItem: string, 
        newSecondaryItemValue: string, 
        newSecondaryItemSubValue?: string,
    ): string | undefined {
        this.secondaryNavigationService.setActiveItem(newSecondaryItemValue);

        let link = `${version}/${mainItem}/${newSecondaryItemValue}`;

        if (!newSecondaryItemSubValue) {
            // If no newSecondaryItemSubValue, try to pick the first one on newSecondaryItem if it exists
            const newSecondaryItem = this.secondaryNavigationService.getNavItems()?.[version]?.[mainItem]
                ?.find((navItem) => navItem.value === newSecondaryItemValue);
            if (!newSecondaryItem) {
                console.log("Secondary Item Value not found in NavItems: ", newSecondaryItemValue);
                return undefined;
            }

            if ((newSecondaryItem?.subItems?.length ?? 0) > 0) {
                const newSecondaryItemSubValue = newSecondaryItem?.subItems?.[0].value;

                this.secondaryNavigationService.setActiveSubItem(newSecondaryItemSubValue);
                link += `/${newSecondaryItemSubValue}`;
            }
        } else {
            this.secondaryNavigationService.setActiveSubItem(newSecondaryItemSubValue);
            link += `/${newSecondaryItemSubValue}`;
        }

        return link;
    }

    private setActiveNavItems(version: string, mainItem: string): void {
        const navItems = this.secondaryNavigationService.getNavItems()?.[version]?.[mainItem];
        this.activeNavItemsSource.next(navItems ?? []);
    }

    // Getters
    public getNavItemsConfiguration(): Record<string, Record<string, UIItem[]>> {
        return this.secondaryNavigationService.getNavItems();
    }
    
    public getNavItems(type: NavigationItemType): UIItem[] {
        switch (type) {
            case NavigationItemType.Version:
                return this.versionService.getVersions();
            case NavigationItemType.MainItem:
                return this.mainNavigationService.getNavItems();
            case NavigationItemType.SecondaryItem:
                return this.getActiveNavItems();
            default:
                return [];
        }
    }

    public getActiveItem(type: NavigationItemType): string {
        switch (type) {
            case NavigationItemType.Version:
                return this.versionService.getActiveVersion();
            case NavigationItemType.MainItem:
                return this.mainNavigationService.getActiveItem();
            case NavigationItemType.SecondaryItem:
                return this.secondaryNavigationService.getActiveItem();
            default:
                return "";
        }
    }

    get activeNavItems$() {
        return this.activeNavItemsSource.asObservable();
    }

    getActiveNavItems(): UIItem[] {
        return this.activeNavItemsSource.value;
    }

    public getActiveSubItem(): string | undefined {
        return this.secondaryNavigationService.getActiveSubItem();
    }

}
