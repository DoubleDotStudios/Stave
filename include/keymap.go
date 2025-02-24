package include

func keys() {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch keypress := msg.String(); keypress {
		case "q":
			return m, tea.Quit
		case "tab":
			m.activeTab = min(m.activeTab+1, len(m.Tabs)-1)
			return m, nil
		case "shift+tab":
			m.activeTab = max(m.activeTab-1, 0)
			return m, nil
		}
	}
}
