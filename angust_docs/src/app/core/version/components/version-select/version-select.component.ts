import { Component } from '@angular/core';
import { VersionService } from '../../services/version.service';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

@Component({
    selector: 'app-version-select',
    standalone: true,
    imports: [CommonModule, FormsModule],
    templateUrl: './version-select.component.html',
    styleUrl: './version-select.component.css',
})
export class VersionSelectComponent {
    versions: string[];
    activeVersion: string;

    constructor(private readonly versionService: VersionService) {
        this.versions = this.versionService.getVersions();
        this.activeVersion = this.versionService.getActiveVersion();
    }

    onVersionChange(newVersion: string): void {
        this.versionService.setActiveVersion(newVersion);
        this.activeVersion = newVersion;
    }
}
