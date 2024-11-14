import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { NavigationManagerService } from '../../services/navigation-manager.service';
import { NavigationItemType } from '../../models/navigation';
import { faCaretDown, faCaretUp } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';

@Component({
    selector: 'app-navigation-sidebar',
    standalone: true,
    imports: [CommonModule, FontAwesomeModule],
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

    faCaretUp = faCaretUp;
    faCaretDown = faCaretDown;
}
