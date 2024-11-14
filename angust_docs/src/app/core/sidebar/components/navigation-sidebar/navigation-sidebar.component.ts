import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { NavigationManagerService } from '../../services/navigation-manager.service';
import { NavigationItemType } from '../../models/navigation';
import { faCaretDown, faCaretUp } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { UIItem } from '../../../../shared/types';

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

    navigateTo(type: NavigationItemType, item: UIItem, subItemValue?: string): void {
        item.isExpanded = true;
        this.navigationManagerService.navigateTo(type, item.value, subItemValue);
    }

    toggleExpand(event: Event, item: UIItem): void {
        event.stopPropagation(); // Prevent navigateTo() from firing.
        item.isExpanded = !item.isExpanded;
    }    

    faCaretUp = faCaretUp;
    faCaretDown = faCaretDown;
}
