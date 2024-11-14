import { Injectable } from "@angular/core";
import { Router } from "@angular/router";
import { VersionService } from "./version.service";
import { SecondaryNavigationService } from "./secondary-navigation.service";
import { MainNavigationService } from "./main-navigation.service";
import { NavigationItemType } from "../models/navigation";
import { UIItem } from "../../../shared/types";

/*
 * Service responsible for managing the three navigation directories.
 */
@Injectable({
    providedIn: 'root',
})
export class NavigationManagerService {

    constructor(
        private readonly router: Router,
        private readonly versionService: VersionService,
        private readonly mainNavigationService: MainNavigationService,
        private readonly secondaryNavigationService: SecondaryNavigationService
    ) {}

    public setActiveItem(value: string, type: NavigationItemType) {
        const version = this.versionService.getActiveVersion();
        const mainItem = this.mainNavigationService.getActiveItem();
        
        let link;

        switch (type) {
            case NavigationItemType.Version: {
                this.versionService.setActiveVersion(value);

                // Ensure to update the active secondary navigation item to the first one when changing the version.
                const firstSecondaryItem = this.secondaryNavigationService.getNavItems()?.[version]?.[mainItem]?.[0];
                if (firstSecondaryItem) {
                    link = `${value}/${mainItem}/${firstSecondaryItem.value}`;
                    this.secondaryNavigationService.setActiveItem(firstSecondaryItem.value);
                    this.secondaryNavigationService.setActiveNavItems(version, mainItem);
                }

                break;
            }
            case NavigationItemType.MainItem: {
                this.mainNavigationService.setActiveItem(value);
                
                // Ensure to update the active secondary navigation item to the first one when changing the main item.
                const firstSecondaryItem = this.secondaryNavigationService.getNavItems()?.[version]?.[value]?.[0];
                if (firstSecondaryItem) {
                    link = `${version}/${value}/${firstSecondaryItem.value}`;
                    this.secondaryNavigationService.setActiveItem(firstSecondaryItem.value);
                    this.secondaryNavigationService.setActiveNavItems(version, value);
                } else {
                    console.log("No secondary items for main item: ", value);
                }

                break;
            }
            case NavigationItemType.SecondaryItem:
                this.secondaryNavigationService.setActiveItem(value);
                link = `${version}/${mainItem}/${value}`;
                break;
            default:
                break;
        }

        if (!link) {
            console.log("Invalid link type: ", type);
            return;
        }
        this.router.navigate([link]);
    }

    public getNavItems(type: NavigationItemType): UIItem[] {
        switch (type) {
            case NavigationItemType.Version:
                return this.versionService.getVersions();
            case NavigationItemType.MainItem:
                return this.mainNavigationService.getNavItems();
            case NavigationItemType.SecondaryItem:
                return this.secondaryNavigationService.getActiveNavItems();
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
}
