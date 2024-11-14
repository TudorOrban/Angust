import { Component } from '@angular/core';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import {
    faQuestion,
} from '@fortawesome/free-solid-svg-icons';

import { ThemeToggleComponent } from '../../../theme/commponents/theme-toggle/theme-toggle.component';
import { VersionSelectComponent } from '../version-select/version-select.component';
import { CommonModule } from '@angular/common';
import { NavigationManagerService } from '../../services/navigation-manager.service';
import { NavigationItemType } from '../../models/navigation';

@Component({
    selector: 'app-main-sidebar',
    standalone: true,
    imports: [
        CommonModule,
        FontAwesomeModule,
        ThemeToggleComponent,
        VersionSelectComponent,
    ],
    templateUrl: './main-sidebar.component.html',
    styleUrl: './main-sidebar.component.css',
})
export class MainSidebarComponent {
    constructor(readonly navigationManagerService: NavigationManagerService) {}

    NavigationItemType = NavigationItemType;

    faQuestion = faQuestion;

}
