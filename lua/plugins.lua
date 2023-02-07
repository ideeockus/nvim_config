-- This file can be loaded by calling `lua require('plugins')` from your init.vim

return require('packer').startup(function(use)
  -- Packer can manage itself
  -- use 'wbthomason/packer.nvim'

  -- Simple plugins can be specified as strings
  use 'rebelot/kanagawa.nvim'
  use 'https://github.com/sainnhe/everforest'
  use 'folke/tokyonight.nvim'

  use 'neovim/nvim-lspconfig'
  use 'simrat39/rust-tools.nvim'

  -- Local plugins can be included
  -- use '~/projects/personal/hover.nvim'

  -- Completion framework:
    use 'hrsh7th/nvim-cmp' 

    -- LSP completion source:
    use 'hrsh7th/cmp-nvim-lsp'

    -- Useful completion sources:
    use 'hrsh7th/cmp-nvim-lua'
    use 'hrsh7th/cmp-nvim-lsp-signature-help'
    use 'hrsh7th/cmp-vsnip'                             
    use 'hrsh7th/cmp-path'                              
    use 'hrsh7th/cmp-buffer'                            
    use 'hrsh7th/vim-vsnip'

    use 'nvim-treesitter/nvim-treesitter'

    use { 'nvim-lualine/lualine.nvim', requires = { 'kyazdani42/nvim-web-devicons', opt = true } }
    use {'akinsho/bufferline.nvim', tag = "v3.*", requires = 'nvim-tree/nvim-web-devicons'}

    use "lukas-reineke/indent-blankline.nvim"

    use "numToStr/Comment.nvim"

    use "nvim-tree/nvim-tree.lua"
    use "lewis6991/gitsigns.nvim"

    use "https://github.com/chrisbra/Colorizer"

end)

