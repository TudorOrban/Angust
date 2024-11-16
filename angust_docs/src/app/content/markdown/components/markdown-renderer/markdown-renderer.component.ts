import {
    Component,
    Input,
    OnChanges,
    OnDestroy,
    SimpleChanges,
} from '@angular/core';
import { MarkdownRendererService } from '../../services/markdown-renderer.service';
import { CommonModule } from '@angular/common';
import { Subscription } from 'rxjs';
import { ThemeService } from '../../../../core/theme/services/theme.service';

@Component({
    selector: 'app-markdown-renderer',
    standalone: true,
    imports: [CommonModule],
    templateUrl: './markdown-renderer.component.html',
    styleUrl: './markdown-renderer.component.css',
})
export class MarkdownRendererComponent implements OnChanges, OnDestroy {
    @Input() fileContent?: string;
    renderedContent?: string;
    isMarkdownInvalid = false;
    isMardownReady = false;
    isDarkTheme = false;
    private readonly themeSubscription: Subscription;

    constructor(
        private readonly markdownRendererService: MarkdownRendererService,
        private readonly themeService: ThemeService
    ) {
        this.themeSubscription = this.themeService
            .getIsDarkTheme()
            .subscribe((isDarkTheme) => {
                this.isDarkTheme = isDarkTheme;
                this.updateTheme();
            });
    }

    ngOnChanges(changes: SimpleChanges): void {
        if (!changes['fileContent'] || !this.fileContent) {
            return;
        }

        const result = this.markdownRendererService.renderMarkdown(this.fileContent);
        if (!(result instanceof Promise)) {
            this.renderedContent = result;
            this.isMardownReady = true;
            return;
        }

        result
            .then((renderedContent) => {
                this.renderedContent = renderedContent;
                this.isMardownReady = true;
            })
            .catch((error) => {
                console.error('Error rendering markdown:', error);
                this.renderedContent = 'Error: Unable to render markdown.';
                this.isMarkdownInvalid = true;
            });
    }

    private updateTheme() {
        this.updateMarkdownTheme();
        this.updateHighlightTheme();
    }
    
    private updateMarkdownTheme() {
        const link = document.createElement('link');
        link.rel ='stylesheet';
        link.href = this.isDarkTheme
           ? 'assets/styles/markdown/github-markdown-dark.css'
           : 'assets/styles/markdown/github-markdown-light.css';
        link.id ='markdown-theme';

        const existingLink = document.getElementById('markdown-theme');
        if (existingLink) {
            document.head.removeChild(existingLink);
        }

        document.head.appendChild(link);
    }

    private updateHighlightTheme() {
        const link = document.createElement('link');
        link.rel = 'stylesheet';
        link.href = this.isDarkTheme
            ? 'assets/styles/markdown/github-dark.css'
            : 'assets/styles/markdown/github.css';
        link.id = 'highlight-theme';

        const existingLink = document.getElementById('highlight-theme');
        if (existingLink) {
            document.head.removeChild(existingLink);
        }

        document.head.appendChild(link);
    }

    ngOnDestroy(): void {
        if (this.themeSubscription) {
            this.themeSubscription.unsubscribe();
        }
    }
}
