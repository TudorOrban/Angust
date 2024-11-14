import { Injectable } from '@angular/core';
import { UIItem } from '../../../shared/types';

@Injectable({
    providedIn: 'root',
})
export class VersionService {
    private readonly versions: UIItem[] = [
        { label: 'v0', value: 'v0' },
        { label: 'v1', value: 'v1' },
    ];
    private activeVersionValue = 'v1';

    constructor() {}

    getVersions(): UIItem[] {
        return this.versions;
    }

    getActiveVersion(): string {
        return this.activeVersionValue;
    }

    setActiveVersion(value: string) {
        this.activeVersionValue = value;
    }
}
