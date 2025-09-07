# Emobanana Web Frontend

This is the Astro-based web frontend for Emobanana, providing the user interface for emoji-based facial expression transformation.

## 🚀 Features

- **React Components**: Built with React for interactive UI components
- **Tailwind CSS**: Modern styling with utility-first CSS framework
- **Image Upload**: Drag-and-drop image upload functionality
- **Real-time Transformation**: Live preview of emoji transformations
- **Responsive Design**: Mobile-first responsive layout

## 🧞 Commands

All commands are run from the `web/` directory:

| Command                   | Action                                           |
| :------------------------ | :----------------------------------------------- |
| `npm install`             | Installs dependencies                            |
| `npm run dev`             | Starts local dev server at `localhost:4321`      |
| `npm run build`           | Build your production site to `./dist/`          |
| `npm run preview`         | Preview your build locally, before deploying     |
| `npm run astro ...`       | Run CLI commands like `astro add`, `astro check` |
| `npm run astro -- --help` | Get help using the Astro CLI                     |

## 📁 Project Structure

```text
web/
├── src/
│   ├── components/
│   │   ├── EmoBananaApp.tsx    # Main application component
│   │   ├── EmojiGrid.tsx       # Emoji selection grid
│   │   ├── ImageUpload.tsx     # Image upload component
│   │   └── TransformResult.tsx # Result display component
│   ├── layouts/
│   │   └── Layout.astro        # Main page layout
│   ├── pages/
│   │   └── index.astro         # Home page
│   └── styles/
│       └── global.css          # Global styles
├── public/                     # Static assets
└── package.json
```

## 🔧 Development

1. Install dependencies:
   ```bash
   npm install
   ```

2. Start development server:
   ```bash
   npm run dev
   ```

3. Open [http://localhost:4321](http://localhost:4321) in your browser

## 📦 Build

Build for production:
```bash
npm run build
```

The built files will be in the `dist/` directory and copied to the backend for deployment.
