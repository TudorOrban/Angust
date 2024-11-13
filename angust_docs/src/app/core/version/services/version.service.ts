import { Injectable } from '@angular/core';

@Injectable({
    providedIn: 'root',
})
export class VersionService {
    private readonly versions = ['v1', 'v2', 'v3'];
    private currentSelectedVersion = 'v1';

    constructor() {}

    getVersions() {
        return this.versions;
    }

    getCurrentSelectedVersion() {
        return this.currentSelectedVersion;
    }

    setCurrentSelectedVersion(version: string) {
        this.currentSelectedVersion = version;
    }
}
