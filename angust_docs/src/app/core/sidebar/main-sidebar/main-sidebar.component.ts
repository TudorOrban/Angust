import { Component } from '@angular/core';
import { ThemeToggleComponent } from "../../theme/commponents/theme-toggle/theme-toggle.component";

@Component({
  selector: 'app-main-sidebar',
  standalone: true,
  imports: [ThemeToggleComponent],
  templateUrl: './main-sidebar.component.html',
  styleUrl: './main-sidebar.component.css'
})
export class MainSidebarComponent {

}
