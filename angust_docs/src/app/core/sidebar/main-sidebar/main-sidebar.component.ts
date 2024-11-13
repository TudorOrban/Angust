import { Component } from '@angular/core';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import {
    faQuestion,
} from '@fortawesome/free-solid-svg-icons';

import { ThemeToggleComponent } from '../../theme/commponents/theme-toggle/theme-toggle.component';
import { VersionSelectComponent } from '../../version/components/version-select/version-select.component';
import { CommonModule } from '@angular/common';
import { MainNavigationService } from '../services/main-navigation.service';

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
    constructor(readonly navigationService: MainNavigationService) {}

    faQuestion = faQuestion;

}
