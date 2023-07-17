## .Cer / .Crt Certificates

To import a self-signed certificate as trusted on Windows, you can use the Microsoft Management Console (MMC) and the Certificate Manager. Here's a step-by-step guide:

1. Open the "Run" dialog by pressing `Win + R` on your keyboard.

2. Type `mmc` and press Enter. This will open the Microsoft Management Console.

3. In the MMC window, go to "File" > "Add/Remove Snap-in".

4. In the "Add or Remove Snap-ins" dialog, select "Certificates" and click "Add".

5. Choose "Computer account" and click "Next".

6. Select "Local computer" and click "Finish".

7. Click "OK" to close the "Add or Remove Snap-ins" dialog.

8. In the MMC window, expand "Certificates (Local Computer)" > "Trusted Root Certification Authorities".

9. Right-click on "Certificates" under "Trusted Root Certification Authorities" and choose "All Tasks" > "Import".

10. Follow the Certificate Import Wizard. Browse and select your self-signed certificate file (usually with the `.cer` or `.crt` extension).

11. Select "Place all certificates in the following store" and click "Browse".

12. Choose "Trusted Root Certification Authorities" and click "OK".

13. Continue clicking "Next" and then "Finish" to complete the import process.

After completing these steps, the self-signed certificate should be imported into the Trusted Root Certification Authorities store on your Windows machine. Applications that rely on the system's trust store, including Rustls and other TLS libraries, should now recognize the certificate as trusted.

---

## Pem Certificates

To import a self-signed PEM certificate as trusted on Windows, you can use the `certutil` command-line tool. Here's a step-by-step guide:

1. Open a Command Prompt window with administrative privileges. To do this, right-click on the "Start" button, choose "Command Prompt (Admin)".

2. Navigate to the directory where your PEM certificate file is located. For example, if your certificate file is in the `C:\certificates` directory, use the following command:

   ```
   cd C:\certificates
   ```

3. Run the following command to import the PEM certificate:

   ```
   certutil -addstore -user "Root" certificate.pem
   ```

   Replace `certificate.pem` with the actual filename of your PEM certificate.

4. After running the command, you should see a message indicating that the certificate was imported successfully.

   ```
   CertUtil: -addstore command completed successfully.
   ```

   The `-addstore` option specifies that you want to add the certificate to the user's certificate store, specifically the "Root" store, which is the Trusted Root Certification Authorities store.

5. Once imported, the self-signed PEM certificate should be trusted by applications that rely on the system's trust store, including TLS libraries in Rust.

Please note that the above instructions import the certificate for the current user. If you need to import the certificate for all users on the machine, you can use the `-user -enterprise` options with `certutil` and run the command in an elevated Command Prompt.

Remember to replace `certificate.pem` in the command with the actual filename and extension of your PEM certificate file.