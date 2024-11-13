import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { MainSidebarComponent } from "./core/sidebar/main-sidebar/main-sidebar.component";
import { NavigationSidebarComponent } from "./core/sidebar/navigation-sidebar/navigation-sidebar.component";

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [RouterOutlet, MainSidebarComponent, NavigationSidebarComponent],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent {
  title = 'angust_docs';
}
