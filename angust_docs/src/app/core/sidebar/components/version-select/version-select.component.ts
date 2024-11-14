import { Component } from '@angular/core';
import { VersionService } from '../../services/version.service';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { UIItem } from '../../../../shared/types';
import { NavigationManagerService } from '../../services/navigation-manager.service';
import { NavigationItemType } from '../../models/navigation';

@Component({
    selector: 'app-version-select',
    standalone: true,
    imports: [CommonModule, FormsModule],
    templateUrl: './version-select.component.html',
    styleUrl: './version-select.component.css',
})
export class VersionSelectComponent {
    versions: UIItem[];
    activeVersionValue: string;

    constructor(
        private readonly navigationManagerService: NavigationManagerService,
    ) {
        this.versions = this.navigationManagerService.getNavItems(NavigationItemType.Version);
        this.activeVersionValue = this.navigationManagerService.getActiveItem(NavigationItemType.Version);
    }

    onVersionChange(newVersion: string): void {
        this.navigationManagerService.setActiveItem(newVersion, NavigationItemType.Version);

        this.activeVersionValue = newVersion;
    }
}
