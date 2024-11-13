import { Component } from '@angular/core';
import { ThemeService } from '../../services/theme.service';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { faMoon } from '@fortawesome/free-solid-svg-icons';

@Component({
    selector: 'app-theme-toggle',
    standalone: true,
    imports: [FontAwesomeModule],
    templateUrl: './theme-toggle.component.html',
    styleUrl: './theme-toggle.component.css',
})
export class ThemeToggleComponent {
    constructor(readonly themeService: ThemeService) {}

    toggleTheme() {
        this.themeService.toggleTheme();
    }

    faMoon = faMoon;
}
