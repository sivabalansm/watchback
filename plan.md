# watchback program
struct File {
    string name;
    string location;
    string checksum;
    tm datetime;
}

Archiver:
- vector<string> files
+Archiver()
+void addFile(struct File file)
+bool archive(string location)
+bool compress()
+bool compressedArchive(string location);
+bool decompress()
+bool decompressedArchive();
it tracks files

AutoArchiver inherits Archiver:
- vector<function<bool(string)>> conditions
+archiveOn(function<bool(string)> conditional)
+void run()
does it track files?
more efficient to use archiver memory or inherits all of archiver



WatchDog:
- directory
- callback
--
+WatchDog(string directory, function<void(string)>callback)
+WatchDog(string directory, function<void>callback)
+watch()
Responsibilites:
Check for a file that has been changed
if file changed, then send its name to callback function (to do anything with it)

AutoArchiver aa;
aa.archiveOn(checkPastSevenDays) // adds to an files list
aa.runWhen(checkSaturday)
callback(file) {
    aa.addFile(file);
    if (aa.runConditionally()) cout << "Currently running backup" << cend;
}
