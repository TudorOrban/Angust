import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { NavigationManagerService } from '../../services/navigation-manager.service';
import { NavigationItemType } from '../../models/navigation';

@Component({
    selector: 'app-navigation-sidebar',
    standalone: true,
    imports: [CommonModule],
    templateUrl: './navigation-sidebar.component.html',
    styleUrl: './navigation-sidebar.component.css',
})
export class NavigationSidebarComponent {
    secondaryNavItems$;
    
    constructor(
        readonly navigationManagerService: NavigationManagerService,
    ) {
        this.secondaryNavItems$ = this.navigationManagerService.activeNavItems$;
    }

    NavigationItemType = NavigationItemType;

}
