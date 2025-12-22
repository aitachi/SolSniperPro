import React from 'react'
import { Github, Twitter, FileText } from 'lucide-react'

export const Footer: React.FC = () => {
  return (
    <footer className="bg-dark-800 border-t border-dark-700 px-6 py-4">
      <div className="flex items-center justify-between">
        <div className="text-sm text-gray-400">
          <p>
            &copy; 2025 SolSniper Pro. Built with{' '}
            <span className="text-danger-500">&hearts;</span> for Solana traders.
          </p>
        </div>

        <div className="flex items-center gap-4">
          <a
            href="https://github.com/solsniper-pro"
            target="_blank"
            rel="noopener noreferrer"
            className="text-gray-400 hover:text-primary-500 transition-colors"
            title="GitHub"
          >
            <Github size={18} />
          </a>
          <a
            href="https://twitter.com/solsniperpro"
            target="_blank"
            rel="noopener noreferrer"
            className="text-gray-400 hover:text-primary-500 transition-colors"
            title="Twitter"
          >
            <Twitter size={18} />
          </a>
          <a
            href="/docs"
            className="text-gray-400 hover:text-primary-500 transition-colors"
            title="Documentation"
          >
            <FileText size={18} />
          </a>
        </div>
      </div>
    </footer>
  )
}
