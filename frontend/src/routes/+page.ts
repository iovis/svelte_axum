import { browser } from "$app/environment";

// client-only code here
if (browser) {
  // Given that I can't see the console in the iPad,
  // show it on the page
  console.log = function (message) {
    const debugNode = window.document.getElementById("debug");

    const node = window.document.createElement("p");
    node.textContent = JSON.stringify(message, null, 2);
    debugNode?.append(node);
  };

  console.error = console.debug = console.info = console.log;
}

// export function load({ params }) {
//     return {
//         user: {
//             title: `Title for ${params.slug} goes here`,
//             content: `Content for ${params.slug} goes here`
//         }
//     };
// }
