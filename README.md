# YamBam

YamBam is a personal kanban board desktop application built for Windows (with macOS/Linux support coming later). Think Trello, but it's a self-contained .exe that runs entirely on your machine - no cloud, no login, no fuss. Just drop the executable in a folder, and it automatically sets up everything you need. It's built for people who want a powerful, feature-rich kanban board without the overhead of web apps or subscription services.

Under the hood, YamBam uses Tauri 2.9 (Rust + web tech) for a lightweight native app (~5-10MB), with a React 18.3 + TypeScript frontend styled with Tailwind CSS and shadcn/ui components. The UI is dark mode only with smooth Framer Motion animations. Each board gets its own SQLite database and file storage folder, keeping everything organized and portable. The tech stack includes @dnd-kit for drag-and-drop, TipTap for rich text editing, Zustand for state management, and TanStack Query for data handling - all the modern stuff that makes development smooth and performant.

Future plans include automation rules (auto-move cards based on triggers), time tracking with billable hours, recurring tasks, templates system, custom fields, calendar views, and potentially a mobile companion app. The goal is to build a feature-complete productivity tool that feels polished and stays completely under your control.
