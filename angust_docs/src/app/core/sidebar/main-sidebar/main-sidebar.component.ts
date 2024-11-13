import { Component } from '@angular/core';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { faAddressBook, faCode, faQuestion, faUser } from '@fortawesome/free-solid-svg-icons';

import { ThemeToggleComponent } from '../../theme/commponents/theme-toggle/theme-toggle.component';
import { VersionSelectComponent } from '../../version/components/version-select/version-select.component';
import { UIItem } from '../../../shared/types';
import { CommonModule } from '@angular/common';

@Component({
    selector: 'app-main-sidebar',
    standalone: true,
    imports: [CommonModule, FontAwesomeModule, ThemeToggleComponent, VersionSelectComponent],
    templateUrl: './main-sidebar.component.html',
    styleUrl: './main-sidebar.component.css',
})
export class MainSidebarComponent {
    mainNavigationItems: UIItem[] = [
        {
            label: "User Guide",
            value: "user-guide",
            icon: faUser,
        },
        {
            label: "Contributor Guide",
            value: "contributor-guide",
            icon: faCode,
        },
        {
            label: "API Reference",
            value: "api-reference",
            icon: faAddressBook,
        }
    ];
    activeItemValue = "user-guide";

    faQuestion = faQuestion;

    setActiveItem(value: string) {
        this.activeItemValue = value;
    }
}
