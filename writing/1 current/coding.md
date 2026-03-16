minimal vable product (3), atechure (4), reference content (5-11), extension tutorial practice (12), publishing managing (13), cross broswer (14), vite framworks (15).

Shadow DOM CSS isolated

Use File Injection when:

✅ Complex logic needed
✅ Want persistent event listeners
✅ Modular, reusable code
Example: Most programmatic injections (RECOMMENDED)

�Injecting Files
Instead of passing in functions and strings, it is also possible to inject 
content scripts defined in files. The following example demonstrates how 
to inject JS and CSS:

Example  8-9a. background.js
```js
chrome.action.onClicked.addListener((tab) => {
  const target = {
    tabId: tab.id,
  };
  chrome.scripting.executeScript({
    target,
    files: ["content-script.js"],
  });
  chrome.scripting.insertCSS({
    target,
    files: ["content-script.css"],
  });
});
```
