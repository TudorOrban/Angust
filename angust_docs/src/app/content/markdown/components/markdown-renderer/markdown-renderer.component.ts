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
    isDarkTheme = false;
    private readonly themeSubscription: Subscription;

    constructor(
        private readonly markdownRendererService: MarkdownRendererService,
        private readonly themeService: ThemeService
    ) {
        this.themeSubscription = this.themeService
            .getIsDarkTheme()
            .subscribe((isDarkTheme) => {
                console.log('Is dark theme:', isDarkTheme);
                this.isDarkTheme = isDarkTheme;
                this.updateHighlightTheme();
            });
    }

    ngOnChanges(changes: SimpleChanges): void {
        if (!changes['fileContent'] || !this.fileContent) {
            return;
        }

        const result = this.markdownRendererService.renderMarkdown(
            this.fileContent
        );
        if (!(result instanceof Promise)) {
            console.log('Result: ', result);
            this.renderedContent = result;
            return;
        }

        result
            .then((renderedContent) => {
                this.renderedContent = renderedContent;
            })
            .catch((error) => {
                console.error('Error rendering markdown:', error);
                this.renderedContent = 'Error: Unable to render markdown.';
            });
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
