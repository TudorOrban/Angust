import { Injectable } from '@angular/core';
import { Route, Router, Routes } from '@angular/router';
import { VersionService } from './version.service';
import { UIItem } from '../../../shared/types';
import { MarkdownComponent } from '../../../content/markdown/markdown/markdown.component';


@Injectable({
    providedIn: 'root',
})
export class DynamicRoutesService {
    constructor(
        private readonly router: Router,
        private readonly versionService: VersionService
    ) {}
    
    public initializeRoutes(
        mainNavItems: UIItem[],
        secondaryNavItemsMap: Record<string, Record<string, UIItem[]>>
    ): void {
        const routes: Routes = [];

        for (const version of this.versionService.getVersions()) {
            for (const mainItem of mainNavItems) {
                const mainPath = `${version.value}/${mainItem.value}`;
                let secondaryNavItems = secondaryNavItemsMap?.[version.value]?.[mainItem.value] ?? [];
                const mainRoute: Route = {
                    path: mainPath,
                    component: MarkdownComponent,
                    children: this.buildChildRoutes(secondaryNavItems)
                };

                routes.push(mainRoute);
            }
        }

        console.log("Routes: ", routes);

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
