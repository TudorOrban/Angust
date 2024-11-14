import { Injectable } from '@angular/core';

@Injectable({
    providedIn: 'root',
})
export class VersionService {
    private readonly versions = ['v1', 'v2', 'v3'];
    private activeVersion = 'v1';

    constructor() {}

    getVersions() {
        return this.versions;
    }

    getActiveVersion() {
        return this.activeVersion;
    }

    setActiveVersion(version: string) {
        this.activeVersion = version;
    }
}
