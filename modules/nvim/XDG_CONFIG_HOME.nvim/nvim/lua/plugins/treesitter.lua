return {
	{
		"nvim-treesitter/nvim-treesitter",
		dependencies = {
			"bezhermoso/tree-sitter-ghostty",
		},
		opts_extend = { "ensure_installed" },
		opts = {
			-- Add custom treesitters not present by default in LazyVim.
			ensure_installed = {
				"csv",
				"dockerfile",
				"embedded_template",
				"ghostty",
				"ini",
				"rust",
				"typescript",
				"toml",
			},
		},
	},
}
