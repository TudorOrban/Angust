import { Injectable } from '@angular/core';
import { Route, Router, Routes } from '@angular/router';
import { VersionService } from '../../version/services/version.service';
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
        secondaryNavItems: UIItem[]
    ): void {
        const routes: Routes = [];

        for (const version of this.versionService.getVersions()) {
            for (const mainItem of mainNavItems) {
                const mainPath = `${version}/${mainItem.value}`;
                const mainRoute: Route = {
                    path: mainPath,
                    component: MarkdownComponent,
                    children: [],
                };

                for (const secondaryItem of secondaryNavItems) {
                    const childRoute: Route = {
                        path: secondaryItem.value,
                        component: MarkdownComponent,
                    };

                    if (!mainRoute.children) {
                        continue;
                    }
                    mainRoute.children.push(childRoute);
                }

                routes.push(mainRoute);
            }
        }

        console.log("Routes: ", routes);

        this.router.resetConfig(routes);
    }
}
