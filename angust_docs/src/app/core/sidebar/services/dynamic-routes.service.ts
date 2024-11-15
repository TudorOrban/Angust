import { Injectable } from '@angular/core';
import { Route, Router, Routes } from '@angular/router';
import { UIItem } from '../../../shared/types';
import { MarkdownComponent } from '../../../content/markdown/components/markdown/markdown.component';
import { NavigationManagerService } from './navigation-manager.service';
import { NavigationItemType } from '../models/navigation';


@Injectable({
    providedIn: 'root',
})
export class DynamicRoutesService {
    constructor(
        private readonly router: Router,
        private readonly navigationManagerService: NavigationManagerService,
    ) {}
    
    public initializeRoutes(): void {
        const versions = this.navigationManagerService.getNavItems(NavigationItemType.Version);
        const mainNavItems = this.navigationManagerService.getNavItems(NavigationItemType.MainItem);
        const navItemsConfiguration = this.navigationManagerService.getNavItemsConfiguration();

        const routes: Routes = [];

        for (const version of versions) {
            for (const mainItem of mainNavItems) {
                const mainPath = `${version.value}/${mainItem.value}`;
                let secondaryNavItems = navItemsConfiguration?.[version.value]?.[mainItem.value] ?? [];

                const mainRoute: Route = {
                    path: mainPath,
                    component: MarkdownComponent,
                    children: this.buildChildRoutes(secondaryNavItems),
                };

                routes.push(mainRoute);
            }
        }

        this.router.resetConfig(routes);
    }

    private buildChildRoutes(items: UIItem[]): Routes {
        let routes: Routes = [];

        items.forEach(item => {
            let route: Route = {
                path: item.value,
                component: MarkdownComponent,
                children: []
            };

            // Check for sub-items and recursively build routes for them
            if (item.subItems && item.subItems.length > 0) {
                route.children = this.buildChildRoutes(item.subItems);
            }

            routes.push(route);
        });

        return routes;
    }
}
