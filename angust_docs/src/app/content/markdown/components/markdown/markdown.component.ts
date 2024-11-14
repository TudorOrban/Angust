import { Component, OnInit } from '@angular/core';
import { NavigationManagerService } from '../../../../core/sidebar/services/navigation-manager.service';
import { NavigationItemType } from '../../../../core/sidebar/models/navigation';
import { MarkdownRendererComponent } from '../markdown-renderer/markdown-renderer.component';
import { HttpClient } from '@angular/common/http';
import { CommonModule } from '@angular/common';

@Component({
    selector: 'app-markdown',
    standalone: true,
    imports: [CommonModule, MarkdownRendererComponent],
    templateUrl: './markdown.component.html',
    styleUrl: './markdown.component.css',
})
export class MarkdownComponent implements OnInit {
    markdownContent?: string;
    isMarkdownFileAbsent = false;

    constructor(
        private readonly navigationManagerService: NavigationManagerService,
        private readonly http: HttpClient,
    ) {}

    ngOnInit(): void {
        const version = this.navigationManagerService.getActiveItem(NavigationItemType.Version);
        const mainItem = this.navigationManagerService.getActiveItem(NavigationItemType.MainItem);
        const secondaryItem = this.navigationManagerService.getActiveItem(NavigationItemType.SecondaryItem);
        const secondarySubItem = this.navigationManagerService.getActiveSubItem();

        const filePath = this.constructMarkdownFilePath(version, mainItem, secondaryItem, secondarySubItem);

        this.loadMarkdownFile(filePath);
    }

    private constructMarkdownFilePath(version: string, mainItem: string, secondaryItem: string, secondarySubItem?: string): string {
        let filePath = `assets/markdown/${version}/${mainItem}/${secondaryItem}`;
        if (secondarySubItem) {
            filePath += `/${secondarySubItem}`;
        }

        return `${filePath}.md`;
    }
    
    private loadMarkdownFile(filePath: string): void {
        this.http.get(filePath, { responseType: 'text' }).subscribe(
            (data) => {
                this.markdownContent = data;
            },
            (error) => {
                console.error('Error loading markdown file:', error);
                this.isMarkdownFileAbsent = true;
            }
        );
    }
}
