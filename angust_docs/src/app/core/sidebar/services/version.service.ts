import { Injectable } from '@angular/core';
import { UIItem } from '../../../shared/types';

@Injectable({
    providedIn: 'root',
})
export class VersionService {
    private readonly versions: UIItem[] = [
        { label: 'v1', value: 'v1' },
        { label: 'v2', value: 'v2' },
        { label: 'v3', value: 'v3' }
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
