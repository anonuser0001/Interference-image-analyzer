use std::borrow::BorrowMut;

use egui::RichText;

use crate::app::AppWindow;

#[derive(PartialEq)]
enum Lang {
    En,
    Sr,
}

pub struct Help {
    show: bool,
    lang: Lang,
}
impl Help {
    pub fn new() -> Help {
        Help {
            show: true,
            lang: Lang::En,
        }
    }
}
impl AppWindow for Help {
    fn show(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) -> Option<Box<dyn AppWindow>> {
        egui::Window::new("Help")
            .open(self.show.borrow_mut())
            .default_width(1000.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Language: ");
                    ui.radio_value(&mut self.lang, Lang::En, "EN");
                    ui.radio_value(&mut self.lang, Lang::Sr, "SR");
                });

                if self.lang==Lang::En {
                    ui.add_space(15.0);
                    ui.label("Interference image analyzer is the program with the help of which the intensity distributions of the interference fringes are obtained. The program scans the image by passing through lines one pixel wide and draws a graph by calculating the intensity of the corresponding color at that pixel. ");
                    ui.add_space(20.0);

                    ui.label(RichText::new("Image processing").heading());
                    ui.separator();
                    ui.add_space(5.0);
                    ui.label("In order for the program to successfully analyze the image, the following conditions must be met:");
                    ui.add_space(5.0);
                    ui.label("• The interference fringes should be horizontal.");
                    ui.add_space(5.0);
                    ui.label("• The interference fringes should be red or green in color.");
                    ui.add_space(5.0);
                    ui.label("• It is desirable for the image to be cropped to contain those fringes that are clearly visible.");
                    ui.add_space(20.0);

                    ui.label(RichText::new("Loading Images").heading());
                    ui.separator();
                    ui.add_space(5.0);
                    ui.label("After launching the program, a window for selecting an interference image is opened.");
                    ui.add_space(5.0);
                    ui.label("• By clicking the Open file button, a window for selecting an image (in .png or .jpg format) from the file system opens.");
                    ui.add_space(5.0);
                    ui.label("• After selecting the image, its preview and the Analyze button appear.");
                    ui.add_space(5.0);
                    ui.label("• By clicking the Analyze button, a new window with intensity distributions of the given image opens.");
                    ui.add_space(20.0);

                    ui.label(RichText::new("Analysis of intensity distributions").heading());
                    ui.separator();
                    ui.add_space(5.0);
                    ui.label("As an output from the program, you will obtain as many graphics as the width of the image in pixels.");
                    ui.add_space(5.0);
                    ui.label("• To display the corresponding intensity distribution, first, the color of the interference fringes should be selected by clicking the Red or Green button in the Color channel row.");
                    ui.add_space(5.0);
                    ui.label("• In the Line index row, by clicking the +/- buttons or dragging the slider, the intensity distribution of the selected vertical line of the input image can be viewed.");
                    ui.add_space(5.0);
                    ui.label("• By clicking the Copy coordinates to clipboard button, located in the top right corner of the Analyze window, the coordinates of the observed intensity distribution are copied for further analysis in the desired program.");
                    ui.add_space(20.0);

                    ui.label(RichText::new("Averaging intensity distributions").heading());
                    ui.separator();
                    ui.add_space(5.0);
                    ui.label("With the help of this program, averaging can be performed to more precisely determine the position of the maximum (center) of the interference lines.");
                    ui.add_space(5.0);
                    ui.label("• To add the observed intensity distribution to the list for averaging, Add to averaged graph button should be clicked.");
                    ui.add_space(5.0);
                    ui.label("• The list of added distributions is located on the right side of the window, with options to show (Show button) or remove (Remove button) distributions from the list.");
                    ui.add_space(5.0);
                    ui.label("• The averaged distribution can be visualized by clicking the Averaging button in the Mode row.");
                    ui.add_space(5.0);
                    ui.label("• Clicking the Copy coordinates to clipboard button will copy the coordinates of the averaged distribution for further use.");
                    ui.add_space(20.0);
                } else {
                    ui.add_space(15.0);
                    ui.label("Interference image analyzer je program pomoću kojeg su dobijene raspodele intenziteta interferencionih pruga. Program skenira sliku tako što prolazi kroz linije širine jednog piksela i crta grafik računajući intenzitet odgovarajuće boje na tom pikselu. ");
                    ui.add_space(20.0);

                    ui.label(RichText::new("Obrada slike").heading());
                    ui.separator();
                    ui.add_space(5.0);
                    ui.label("Da bi program uspešno analizirao sliku potrebno je da budu ispunjeni sledeći uslovi:");
                    ui.add_space(5.0);
                    ui.label("• Interferencione pruge treba da budu horizontalne");
                    ui.add_space(5.0);
                    ui.label("• Interferencione pruge treba da budu crvene ili zelene boje");
                    ui.add_space(5.0);
                    ui.label("• Poželjno je da slika bude isečena tako da sadrzi one pruge koje su jasno vidljive");
                    ui.add_space(20.0);

                    ui.label(RichText::new("Učitavanje slika").heading());
                    ui.separator();
                    ui.add_space(5.0);
                    ui.label("Nakon paljenja programa prisutan je prozor za odabir interferencione slike.");
                    ui.add_space(5.0);
                    ui.label("• Klikom na dugme Open file otvara se prozor za odabir slike na fajl sistemu u .png ili .jpg formatu");
                    ui.add_space(5.0);
                    ui.label("• Nakon odabira slike pojavljuje se njen prikaz i dugme Analyze");
                    ui.add_space(5.0);
                    ui.label("• Klikom na dugme Analyze otvara se novi prozor sa raspodelama intenziteta date slike");
                    ui.add_space(20.0);

                    ui.label(RichText::new("Analiza raspodela intenziteta").heading());
                    ui.separator();
                    ui.add_space(5.0);
                    ui.label("Kao izlaz iz programa dobija se onoliki broj grafika kolika je širina slike u pikselima.");
                    ui.add_space(5.0);
                    ui.label("• Prvo što je potrebno uraditi kako bi se prikazala odovarajuca raspodela intenziteta jeste da se izabere boja interferencionih pruga klikom na dugme Red ili Green u redu Color channel");
                    ui.add_space(5.0);
                    ui.label("• U redu Line index klikom na dugme +/- ili prevlacenjem slajdera dobija se prikaz raspodele intenziteta izabrane vertikale unesene slike");
                    ui.add_space(5.0);
                    ui.label("• Klikom na dugme Copy coordinates to clipboard koje se nalazi u gornjem desnom uglu prozora Analyze, koordinate posmatrane raspodele se kopiraju radi dalje analize u željenom programu");
                    ui.add_space(20.0);

                    ui.label(RichText::new("Usrednjavanje raspodela intenziteta").heading());
                    ui.separator();
                    ui.add_space(5.0);
                    ui.label("Pomoću ovog programa je izvršeno usrednjavanje sa ciljem preciznijeg određivanja položaja maksimuma (centara) interferencionih pruga.");
                    ui.add_space(5.0);
                    ui.label("• Dodavanje posmatrane raspodele u listu za usrednjavanje se postize klikom na dugme Add to averaged graph");
                    ui.add_space(5.0);
                    ui.label("• Lista dodatih raspodela nalazi se sa desne strane prozora, uz mogućnost prikazivanja (dugme Show) ili uklanjanja  (dugme Remove) raspodela iz liste");
                    ui.add_space(5.0);
                    ui.label("• Usrednjena raspodela moze  se videti klikom na dugme Averaging, u redu Mode");
                    ui.add_space(5.0);
                    ui.label("• Koordinate usrednjene raspodele takodje se kopiraju klikom na dugme Copy coordinates to clipboard");
                    ui.add_space(20.0);
                }
            });

        return None;
    }

    fn get_visibility(&self) -> bool {
        self.show
    }
}
