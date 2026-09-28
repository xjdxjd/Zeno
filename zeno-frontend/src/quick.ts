import './app.css'
import QuickSearch from './lib/QuickSearch.svelte'
import { mount } from 'svelte'

const app = mount(QuickSearch, { target: document.getElementById('app')! })

export default app
