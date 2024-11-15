import { RouteReuseStrategy, DetachedRouteHandle, ActivatedRouteSnapshot } from '@angular/router';
import { MarkdownComponent } from '../../content/markdown/components/markdown/markdown.component';

export class CustomRouteReuseStrategy implements RouteReuseStrategy {
    shouldDetach(route: ActivatedRouteSnapshot): boolean {
        return false;
    }

    store(route: ActivatedRouteSnapshot, handle: DetachedRouteHandle | null): void { /* */ }

    shouldAttach(route: ActivatedRouteSnapshot): boolean {
        return false;
    }

    retrieve(route: ActivatedRouteSnapshot): DetachedRouteHandle | null {
        return null;
    }

    shouldReuseRoute(future: ActivatedRouteSnapshot, curr: ActivatedRouteSnapshot): boolean {
        if (future.component === MarkdownComponent || curr.component === MarkdownComponent) {
            return false; // Prevent reuse for MarkdownComponent (page content)
        }

        return future.routeConfig === curr.routeConfig; // Allow reuse for other components
    }
}
