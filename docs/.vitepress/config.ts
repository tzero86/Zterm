import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'Zterm',
  description: 'Open-source terminal with built-in local AI — no login, no paywalls.',
  base: '/Zterm/',

  head: [
    ['link', { rel: 'icon', href: '/Zterm/favicon.ico' }],
  ],

  themeConfig: {
    logo: '/logo.svg',
    siteTitle: 'Zterm',

    nav: [
      { text: 'Guide', link: '/guide/getting-started' },
      { text: 'AI Features', link: '/ai/local-llm' },
      { text: 'GitHub', link: 'https://github.com/tzero86/Zterm' },
    ],

    sidebar: [
      {
        text: 'Getting Started',
        items: [
          { text: 'Introduction', link: '/guide/introduction' },
          { text: 'Installation', link: '/guide/getting-started' },
          { text: 'Configuration', link: '/guide/configuration' },
        ],
      },
      {
        text: 'AI Features',
        items: [
          { text: 'Local LLM Setup', link: '/ai/local-llm' },
          { text: 'Ollama', link: '/ai/ollama' },
          { text: 'LM Studio', link: '/ai/lmstudio' },
          { text: 'Agent Mode', link: '/ai/agent-mode' },
        ],
      },
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/tzero86/Zterm' },
    ],

    footer: {
      message: 'Released under the MIT / AGPL License.',
      copyright: 'Copyright © 2026 Zterm Contributors',
    },

    editLink: {
      pattern: 'https://github.com/tzero86/Zterm/edit/main/docs/:path',
      text: 'Edit this page on GitHub',
    },
  },
})
