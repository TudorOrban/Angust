import { Injectable } from '@angular/core';
import { BehaviorSubject } from 'rxjs';

@Injectable({
    providedIn: 'root',
})
export class ThemeService {
    private readonly isDarkThemeSubject = new BehaviorSubject<boolean>(false);

    constructor() {
        const isDarkTheme = localStorage.getItem('dark-theme') === 'true';
        this.isDarkThemeSubject.next(isDarkTheme);
        this.applyTheme(isDarkTheme);
    }

    toggleTheme() {
        const isDarkTheme = !this.isDarkThemeSubject.value;
        this.isDarkThemeSubject.next(isDarkTheme);
        localStorage.setItem('dark-theme', isDarkTheme.toString());
        this.applyTheme(isDarkTheme);
    }

    getIsDarkTheme() {
        return this.isDarkThemeSubject.asObservable();
    }

    getIsDarkThemeValue() {
        return this.isDarkThemeSubject.value;
    }

    private applyTheme(isDarkTheme: boolean) {
        if (isDarkTheme) {
            document.body.classList.add('dark-theme');
        } else {
            document.body.classList.remove('dark-theme');
        }
    }
}
