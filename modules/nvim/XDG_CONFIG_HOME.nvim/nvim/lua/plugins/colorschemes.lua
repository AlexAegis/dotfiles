return {
  {
    "folke/tokyonight.nvim",
    opts = {
      transparent = true,
      styles = {
        comments = { italic = false },
        keywords = { italic = false },
        sidebars = "normal",
        floats = "transparent",
      },
      on_highlights = function(hl, c)
        hl.SnacksPickerPathHidden = { fg = c.comment }
        hl.SnacksPickerDir = { fg = c.comment }
        hl.SnacksPickerGitStatusIgnored = { fg = c.nontext }
        hl.SnacksPickerGitStatusUntracked = { fg = c.nontext }
        hl.ColorColumn = { bg = c.bg_highlight }
        hl.SpellBad = { fg = c.red, undercurl = true }
        hl.SpellCap = { fg = c.red, undercurl = true }
        hl.SpellLocal = { fg = c.red, undercurl = true }
        hl.SpellRare = { fg = c.red, undercurl = true }
      end,
    },
  },
  {
    "rebelot/kanagawa.nvim",
    opts = {
      transparent = true,
    },
  },
}
