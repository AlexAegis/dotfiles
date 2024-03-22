vim.g.gruvbox_material_background = "hard"
vim.g.gruvbox_material_transparent_background = 1

-- For transparency
require("notify").setup({
	background_colour = "#000000"
});

return {
	-- add gruvbox
	{ "sainnhe/gruvbox-material" },
	-- Configure LazyVim to load gruvbox
	{
		"LazyVim/LazyVim",
		opts = {
			colorscheme = "gruvbox-material",
		},
	},
}
