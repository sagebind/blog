<div class="content style-default">
    <header class="main-heading">
        <h1 class="container">Contact</h1>
    </header>
    <?php if (isset($successMessage)): ?>
        <div class="container message success"><?=$successMessage?></div>
    <?php elseif (isset($errorMessage)): ?>
        <div class="message error"><?=$errorMessage?></div>
    <?php endif; ?>

    <div class="container">
        <p>Want to contact me? Send me a message using the beautifully crafted form below.</p>
    </div>

    <form id="contactForm" name="contact" action="/contact/post" method="post">
        <fieldset>
            <label for="nameTextBox">Your name</label>
            <input type="text" id="nameTextBox" name="name" placeholder="John Doe" required>
            <small>What... is your name?</small>
        </fieldset>

        <fieldset>
            <label for="companyTextBox">Company / Organization</label>
            <input type="text" id="companyTextBox" name="company" placeholder="ACME Products, Inc.">
            <small>Are you representing a corporation or organization?</small>
        </fieldset>

        <fieldset>
            <label for="emailTextBox">Email address</label>
            <input type="email" id="emailTextBox" name="email" placeholder="youremail@example.com" required>
            <small>I can't reply to you if you don't add this.</small>
        </fieldset>

        <fieldset>
            <label for="contentTextBox">Message content</label>
            <textarea id="contentTextBox" name="content" rows="7" placeholder="Message"></textarea>
            <small class="form-note">
                HTML formatting will be removed. You can format your message using <a href="http://daringfireball.net/projects/markdown/basics" target="_blank">Markdown</a>.
            </small>
        </fieldset>

        <div class="form-buttons">
            <input type="submit" value="Send Message">
            <input type="reset" value="Start Over">
        </div>
    </form>
</div>
