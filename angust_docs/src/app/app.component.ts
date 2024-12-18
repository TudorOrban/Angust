import { Component, OnInit } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { MainSidebarComponent } from './core/sidebar/components/main-sidebar/main-sidebar.component';
import { NavigationSidebarComponent } from './core/sidebar/components/navigation-sidebar/navigation-sidebar.component';
import { DynamicRoutesService } from './core/sidebar/services/dynamic-routes.service';

@Component({
    selector: 'app-root',
    standalone: true,
    imports: [RouterOutlet, MainSidebarComponent, NavigationSidebarComponent],
    templateUrl: './app.component.html',
    styleUrl: './app.component.css',
})
export class AppComponent implements OnInit {
    constructor(
        private readonly dynamicRoutesService: DynamicRoutesService,
    ) {}

    ngOnInit() {
        this.dynamicRoutesService.initializeRoutes();
    }
}
