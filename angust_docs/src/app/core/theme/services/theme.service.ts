import { Injectable } from '@angular/core';

@Injectable({
    providedIn: 'root',
})
export class ThemeService {
    private isDarkTheme = false;

    constructor() {
        this.isDarkTheme = localStorage.getItem('dark-theme') === 'true';
        this.applyTheme();
    }

    toggleTheme() {
        this.isDarkTheme = !this.isDarkTheme;
        localStorage.setItem('dark-theme', this.isDarkTheme.toString());
        this.applyTheme();
    }

    private applyTheme() {
        if (this.isDarkTheme) {
            document.body.classList.add('dark-theme');
        } else {
            document.body.classList.remove('dark-theme');
        }
    }
}
